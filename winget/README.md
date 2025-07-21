# Windows Package Manager (WinGet) Manifests

This directory contains WinGet manifest templates for submitting Biome to the Windows Package Manager Community Repository.

## Prerequisites

Before using the automated WinGet workflow, you need:

1. **Fork the winget-pkgs repository**:
   - Fork [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs) under the same GitHub account/organization as this repository
   - This is required by GitHub's pull request model and Microsoft's submission process

2. **Create a Personal Access Token (PAT)**:
   - Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Create a classic token with `public_repo` scope
   - Add it as a repository secret named `WINGET_TOKEN`

3. **Initial package submission**:
   - At least one version of BiomeJS.Biome must already exist in the WinGet Community Repository
   - This is a one-time manual process (see Manual Submission section below)

## Automated Updates

The automated workflow in `.github/workflows/winget.yml` handles updates automatically when new releases are published.

### How It Works

The automation will:
- Trigger on new releases tagged with `@biomejs/biome@*`
- Extract the version from the release tag  
- Use the [winget-releaser](https://github.com/vedantmgoyal9/winget-releaser) action to:
  - Automatically detect release assets (biome-win32-x64.exe and biome-win32-arm64.exe)
  - Create/update the WinGet manifest
  - Submit a pull request from your fork to microsoft/winget-pkgs

### Current Workflow Configuration

```yaml
- name: Publish to WinGet
  uses: vedantmgoyal9/winget-releaser@v2
  with:
    identifier: BiomeJS.Biome
    version: ${{ steps.extract-version.outputs.version }}
    token: ${{ secrets.WINGET_TOKEN }}
```

## Manual Submission Process (One-time Setup)

### Prerequisites for Manual Submission

1. Install the [WinGet CLI](https://github.com/microsoft/winget-cli)
2. Install one of the manifest creation tools:
   - [winget-create](https://github.com/microsoft/winget-create) - Microsoft's official tool
   - [Komac](https://github.com/russellbanks/komac) - Community tool with enhanced features

### Method 1: Using wingetcreate.exe

The recommended approach is to use `wingetcreate.exe` to automatically generate and submit the manifests:

1. **For creating a new package** (first-time submission):
   ```bash
   wingetcreate.exe new -o ./winget
   ```

2. **For updating an existing package**:
   ```bash
   # Update and submit to GitHub in one command
   wingetcreate.exe update --submit --token <GitHubPersonalAccessToken> --urls https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-x64.exe https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-arm64.exe --version VERSION BiomeJS.Biome
   ```

3. **For local testing** (generates manifests to a directory without submitting):
   ```bash
   # Generate manifests locally for review
   wingetcreate.exe update --out ./manifests --urls https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-x64.exe https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-arm64.exe --version VERSION BiomeJS.Biome
   ```

4. **For submitting locally generated manifests** (once the team is happy with the manifests):
   ```bash
   # Submit manifests that were generated locally
   wingetcreate.exe submit --prtitle <PullRequestTitle> --token <GitHubPersonalAccessToken>
   
   # Use --replace to replace an existing manifest from the Windows Package Manager repo
   wingetcreate.exe submit --prtitle <PullRequestTitle> --token <GitHubPersonalAccessToken> --replace
   ```

### Method 2: Using Komac (Recommended Alternative)

[Komac](https://github.com/russellbanks/komac) is a powerful community-developed alternative that offers several advantages over the official tools:

**Why choose Komac:**
- 🔄 **Advanced installer analysis** - Better detection and metadata extraction from Inno Setup, Nullsoft, MSI, and Burn installers
- 🌍 **Cross-platform support** - Works on Windows, Linux, and macOS
- ⚡ **Better user experience** - Download progress bars, faster processing, and more intelligent automation
- 📊 **Enhanced GitHub integration** - Automatically extracts release notes, dates, and metadata from GitHub releases
- 🔒 **Privacy-focused** - No telemetry (unlike wingetcreate which has telemetry enabled by default)

**Installation:**
```bash
# Install via WinGet
winget install komac

# Or via Cargo (cross-platform)
cargo install --locked komac
```

**Usage:**
```bash
# 1. Set up your GitHub token (one-time setup)
komac token add

# 2. Create a new package
komac new

# 3. Update an existing package with automatic submission
komac update BiomeJS.Biome --version VERSION --urls https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-x64.exe https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-arm64.exe --submit

# 4. Update package without submitting (for review)
komac update BiomeJS.Biome --version VERSION --urls https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-x64.exe https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-arm64.exe
```

**Important Note**: All WinGet submission methods (including wingetcreate.exe and winget-releaser) require a fork of microsoft/winget-pkgs due to GitHub's pull request model. The tools automatically create branches in your fork and submit pull requests to the upstream repository.

## Package Information

- **Package Identifier**: `BiomeJS.Biome`
- **Publisher**: Biome Developers and Contributors
- **Package Name**: Biome
- **License**: MIT OR Apache-2.0
- **Supported Architectures**: x64, arm64
- **Installation Type**: Portable executable (single binary)

### Testing Locally

Before submitting, you can test the manifests generated by wingetcreate.exe:

You will need to run `winget settings --enable LocalManifestFiles` in an administrative shell before working with local manifests.

```bash
# First, generate manifests locally for testing
wingetcreate.exe update --out ./manifests --urls https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-x64.exe https://github.com/biomejs/biome/releases/download/@biomejs/biome@VERSION/biome-win32-arm64.exe --version VERSION BiomeJS.Biome

# Validate the generated manifests
winget validate manifests/b/BiomeJS/Biome/VERSION/

# Test installation (if the package is already in WinGet)
winget install --manifest manifests/b/BiomeJS/Biome/VERSION/
```

## Resources

- [WinGet Package Manager Documentation](https://docs.microsoft.com/en-us/windows/package-manager/)
- [WinGet Manifest Schema](https://github.com/microsoft/winget-cli/tree/master/schemas)
- [WinGet Community Repository](https://github.com/microsoft/winget-pkgs)
- [WinGet Create Tool](https://github.com/microsoft/winget-create)
- [WinGet Releaser Action](https://github.com/vedantmgoyal9/winget-releaser)
