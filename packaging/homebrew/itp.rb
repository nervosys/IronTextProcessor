class Itp < Formula
  desc "Iron Text Processor — an agentic-first successor to grep, sed, and awk"
  homepage "https://github.com/nervosys/IronTextProcessor"
  version "1.1.0"
  license "AGPL-3.0-only"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/nervosys/IronTextProcessor/releases/download/v#{version}/itp-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_AARCH64_DARWIN"
    else
      url "https://github.com/nervosys/IronTextProcessor/releases/download/v#{version}/itp-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64_DARWIN"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/nervosys/IronTextProcessor/releases/download/v#{version}/itp-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_AARCH64_LINUX"
    else
      url "https://github.com/nervosys/IronTextProcessor/releases/download/v#{version}/itp-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_64_LINUX"
    end
  end

  def install
    bin.install "itp"
    bin.install "itp-grep" if File.exist?("itp-grep")
    bin.install "itp-sed" if File.exist?("itp-sed")
    bin.install "itp-awk" if File.exist?("itp-awk")

    # Install shell completions if present
    bash_completion.install "completions/itp.bash" if File.exist?("completions/itp.bash")
    zsh_completion.install "completions/_itp" if File.exist?("completions/_itp")
    fish_completion.install "completions/itp.fish" if File.exist?("completions/itp.fish")

    # Install man page if present
    man1.install "man/itp.1" if File.exist?("man/itp.1")
  end

  test do
    assert_match "itp", shell_output("#{bin}/itp --version")

    # Basic grep test
    (testpath/"test.txt").write("hello world\nfoo bar\nhello again\n")
    output = shell_output("#{bin}/itp search 'hello' #{testpath}/test.txt --format json")
    assert_match "hello", output

    # Basic AQL test
    output = shell_output("echo 'hello world' | #{bin}/itp query 'find \"hello\"'")
    assert_match "hello", output
  end
end
