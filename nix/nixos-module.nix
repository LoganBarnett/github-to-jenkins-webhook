{ lib, pkgs, config, ... }:
let
  inherit (lib) mkIf mkOption types optionalString;
in {
  options.services.github-to-jenkins-webhook = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable the github-to-jenkins-webhook service.";
    };

    package = mkOption {
      type = types.package;
      default = pkgs.github-to-jenkins-webhook;
      description = "Package providing the github-to-jenkins-webhook binary.";
    };

    githubSecretFile = mkOption {
      type = types.path;
      example = "/run/agenix/github_webhook_secret";
      description = ''
        Path to the GitHub webhook secret file. This is loaded with systemd
        LoadCredential and exposed to the service as
        GITHUB_SECRET_FILE=/run/credentials/%n/github_secret_file.
        Avoid builtins.readFile; pass the file path (e.g., from agenix).
      '';
    };

    jenkinsUrl = mkOption {
      type = types.str;
      example = "https://jenkins.example.com/github-webhook/";
      description = ''
        Jenkins endpoint for forwarding events. Exposed via --jenkins-url
        and JENKINS_URL so either CLI or env works.
      '';
    };

    host = mkOption {
      type = types.str;
      default = "127.0.0.1";
      description = "Host/IP to bind. Passed to --host.";
    };

    port = mkOption {
      type = types.port;
      default = 8080;
      description = "TCP port to listen on. Passed to --port (default 8080).";
    };

    environmentFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = ''
        Optional EnvironmentFile with extra variables. Useful to inject
        JENKINS_URL or other settings without rebuilding. Secrets can be
        referenced here (paths), but prefer LoadCredential for the secret file.
      '';
    };

    extraEnvironment = mkOption {
      type = types.attrsOf types.str;
      default = {};
      description = "Additional Environment= KEY=VALUE entries.";
    };

    extraArgs = mkOption {
      type = types.listOf types.str;
      default = [];
      description = "Extra CLI flags.";
    };
  };

  config = mkIf config.services.github-to-jenkins-webhook.enable
    (let
      cfg = config.services.github-to-jenkins-webhook;
      bin = lib.getExe cfg.package;
      args = [
        "--host" cfg.host
        "--port" (toString cfg.port)
        "--jenkins-url" cfg.jenkinsUrl
        "--github-secret-file" "/run/credentials/%n/github_secret_file"
      ] ++ cfg.extraArgs;
    in {
      systemd.services.github-to-jenkins-webhook = {
        description = "GitHub -> Jenkins webhook relay";
        after = [ "network-online.target" ];
        wants = [ "network-online.target" ];
        wantedBy = [ "multi-user.target" ];

        serviceConfig = {
          ExecStart = lib.escapeShellArgs ([ bin ] ++ args);

          Environment = lib.mapAttrsToList
            (k: v: "${k}=${v}")
            cfg.extraEnvironment
          ;
          EnvironmentFile = lib.mkIf (cfg.environmentFile != null) cfg.environmentFile;
          # LoadCredential puts the file at /run/credentials/%n/<name>
          LoadCredential = [ "github_secret_file:${cfg.githubSecretFile}" ];
          # Sandboxing; keep it reasonable for a small HTTP service
          DynamicUser = true;
          ProtectSystem = "strict";
          ProtectHome = true;
          PrivateTmp = true;
          PrivateDevices = true;
          NoNewPrivileges = true;
          LockPersonality = true;
          RestrictRealtime = true;
          RestrictSUIDSGID = true;
          SystemCallArchitectures = "native";
          CapabilityBoundingSet = "";
          AmbientCapabilities = "";
          # Needs network egress to reach Jenkins; do not IPAddressDeny.
          Restart = "on-failure";
          RestartSec = "2s";
        };
      };
    });
}
