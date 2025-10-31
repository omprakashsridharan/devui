# Publishing Guide

This guide explains how to publish the DevUI library to crates.io.

## Prerequisites

1. **Crates.io Account**: Create an account at [crates.io](https://crates.io)
2. **API Token**: Generate a token at https://crates.io/me
3. **GitHub Secrets**: Add the token as `CRATES_IO_TOKEN` in your GitHub repository secrets

## Publishing Process

### Option 1: Automated Publishing (Recommended)

#### Using GitHub Release

1. Create a new release on GitHub
2. Tag the release (e.g., `v0.1.0`)
3. The GitHub workflow will automatically:
   - Build the frontend
   - Build and test the Rust library
   - Publish to crates.io
   - Create a git tag

#### Using Manual Workflow Dispatch

1. Go to Actions → Publish to Crates.io
2. Click "Run workflow"
3. Enter the version number (e.g., `0.1.0`)
4. The workflow will:
   - Update `Cargo.toml` version
   - Build and publish
   - Create a git tag

### Option 2: Manual Publishing

1. **Build Frontend**:
   ```bash
   cd frontend
   npm ci
   npm run build
   cd ..
   ```

2. **Verify Build**:
   ```bash
   # Ensure frontend/dist exists and is populated
   ls -la frontend/dist
   ```

3. **Update Version**:
   ```bash
   # Edit Cargo.toml and update version
   # e.g., version = "0.1.0"
   ```

4. **Run Tests**:
   ```bash
   cargo test --release
   ```

5. **Verify Package**:
   ```bash
   cargo package --list
   ```

6. **Publish**:
   ```bash
   # First time - set token
   cargo login <your-token>

   # Publish
   cargo publish
   ```

## Version Management

- Follow [Semantic Versioning](https://semver.org/)
- Update version in `Cargo.toml` before publishing
- Create a git tag for each release

## What Gets Published

The published package includes:

- ✅ Rust source code (`src/`)
- ✅ Built frontend (`frontend/dist/`)
- ✅ `Cargo.toml`
- ✅ `README.md`
- ✅ `LICENSE` (if exists)

The following are **excluded**:

- ❌ Frontend source files (`frontend/src/`)
- ❌ Node modules (`frontend/node_modules/`)
- ❌ Frontend build config (`frontend/package.json`, `vite.config.ts`, etc.)
- ❌ Examples (`examples/`)
- ❌ GitHub workflows (`.github/`)
- ❌ Documentation files (`docs/`)
- ❌ Development scripts (`Makefile`, `dev.sh`)

## Post-Publishing

1. **Verify Publication**:
   - Check https://crates.io/crates/devui
   - Verify the version is published
   - Check documentation is available

2. **Update Documentation**:
   - Ensure `README.md` is accurate
   - Update any version references
   - Add changelog entry (if you maintain one)

3. **Tag Release** (if not done automatically):
   ```bash
   git tag -a "v0.1.0" -m "Release version 0.1.0"
   git push origin "v0.1.0"
   ```

## Troubleshooting

### Build Failures

- **Frontend build fails**: Ensure Node.js 18+ is installed and dependencies are up to date
- **Rust build fails**: Check that all dependencies compile and tests pass

### Publishing Failures

- **Token issues**: Verify `CRATES_IO_TOKEN` secret is set correctly
- **Version conflicts**: Ensure the version doesn't already exist on crates.io
- **Package too large**: Check that unnecessary files are excluded

### Frontend Not Included

- Ensure `frontend/dist` is built before publishing
- Check that `frontend/dist` is not in `.gitignore`
- Verify the workflow builds the frontend before packaging

## Best Practices

1. **Test Locally First**: Always test with `cargo package` before publishing
2. **Incremental Versions**: Start with `0.1.0` for initial release
3. **Documentation**: Keep `README.md` and `SQL_POSTGRES_FEATURES.md` up to date
4. **Changelog**: Consider maintaining a `CHANGELOG.md` for version history
5. **CI/CD**: Use automated workflows for consistent publishing

## Security Notes

- **Never commit tokens**: Use GitHub secrets for CI/CD
- **Review dependencies**: Regularly update dependencies for security patches
- **Audit packages**: Use `cargo audit` to check for vulnerabilities

## Next Steps After Publishing

1. Share the crate on:
   - Reddit (r/rust)
   - Rust Discord
   - Twitter/X
   - Your blog or website

2. Monitor:
   - Download statistics on crates.io
   - Issues and feature requests
   - Community feedback

3. Maintain:
   - Regular updates
   - Bug fixes
   - Feature additions based on feedback

