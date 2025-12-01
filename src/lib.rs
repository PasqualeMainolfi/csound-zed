use zed_extension_api as zed;
use std::fs;

struct CsoundExtension {
    cached_binary_path: Option<String>
}

impl zed::Extension for CsoundExtension {
    fn new() -> Self
        where
            Self: Sized {
                Self { cached_binary_path: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree
    ) -> zed::Result<zed::Command> {

        let git_repo = "PasqualeMainolfi/csound-lsp";

        let (platform, arch) = zed::current_platform();
        let binary_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "csound-lsp-macos-aarch64",
            (zed::Os::Mac, zed::Architecture::X8664) => "csound-lsp-macos-x86_64",
            (zed::Os::Linux, zed::Architecture::X8664) => "csound-lsp-linux-x86_64",
            (zed::Os::Windows, zed::Architecture::X8664) => "csound-lsp-windows-x86_64.exe",
            _ => return Err(format!("Unsupported platform: {:?} {:?}", platform, arch).into()),
        };

        let server_path = format!("csound-lsp-binary-{}", binary_name);
        if !fs::metadata(&server_path).map(|m| m.is_file()).unwrap_or(false) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let release = zed::latest_github_release(
                git_repo,
                zed::GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )?;

            let asset_url = format!(
                "https://github.com/{}/releases/download/{}/{}",
                git_repo, release.version, binary_name
            );

            eprintln!("ZED: Download  LSP from: {}", asset_url);
            zed::download_file(&asset_url, &server_path, zed::DownloadedFileType::Uncompressed)?;

            // Rendi eseguibile (importante su Mac/Linux)
            zed::make_file_executable(&server_path)?;
        }

        self.cached_binary_path = Some(server_path.clone());

        Ok(zed::Command {
            command: server_path.to_string(),
            args: vec![],
            env: Default::default()
        })
    }

}

zed::register_extension!(CsoundExtension);
