class clearedforpush < Formula
  desc "Pre-push merge conflict predictor with beautiful CLI"
  homepage "https://github.com/sanjayrohith/clearedforpush"
  version "0.1.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_intel do
      url "https://github.com/sanjayrohith/clearedforpush/releases/download/v#{version}/clearedforpush-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_arm do
      url "https://github.com/sanjayrohith/clearedforpush/releases/download/v#{version}/clearedforpush-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    url "https://github.com/sanjayrohith/clearedforpush/releases/download/v#{version}/clearedforpush-v#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "PLACEHOLDER"
  end

  depends_on "git" => ">= 2.38"

  def install
    bin.install "clearedforpush"
  end

  test do
    assert_match "Pre-push merge conflict predictor", shell_output("#{bin}/clearedforpush --help")
  