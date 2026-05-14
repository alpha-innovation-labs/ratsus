# Default: Show help menu
default:
    @just help

# ============================================================================
# Help Command
# ============================================================================

help:
    @echo ""
    @echo "\033[1;36m======================================\033[0m"
    @echo "\033[1;36m          Ratsus Commands             \033[0m"
    @echo "\033[1;36m======================================\033[0m"
    @echo ""
    @echo "\033[1;35m  Most Common Commands:\033[0m"
    @echo "  just \033[0;33mdev\033[0m                     \033[0;32mRun the Nexus TUI wrapper\033[0m"
    @echo "  just \033[0;33mdev-stub\033[0m                \033[0;32mRun the TUI with deterministic stub data\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                   \033[0;32mRun fmt, cargo check, clippy, and tests\033[0m"
    @echo "  just \033[0;33mtest\033[0m                    \033[0;32mRun the test suite with nextest\033[0m"
    @echo ""
    @echo "\033[1;35m  Development:\033[0m"
    @echo "  just \033[0;33mdev\033[0m                     \033[0;32mRun the Nexus TUI wrapper\033[0m"
    @echo "  just \033[0;33mdev-stub\033[0m                \033[0;32mRun the TUI with deterministic stub data\033[0m"
    @echo ""
    @echo "\033[1;35m  Building:\033[0m"
    @echo "  just \033[0;33mrelease\033[0m                 \033[0;32mInstall optimized ratsus binary locally\033[0m"
    @echo ""
    @echo "\033[1;35m  Verification:\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                   \033[0;32mRun fmt, cargo check, clippy, and tests\033[0m"
    @echo "  just \033[0;33mcargo-check\033[0m             \033[0;32mCompile all Rust targets with sccache\033[0m"
    @echo "  just \033[0;33mclippy\033[0m                  \033[0;32mRun Clippy with warnings denied\033[0m"
    @echo "  just \033[0;33mlint\033[0m                    \033[0;32mAlias for Clippy\033[0m"
    @echo "  just \033[0;33mfmt-check\033[0m               \033[0;32mCheck Rust formatting\033[0m"
    @echo ""
    @echo "\033[1;35m  Testing:\033[0m"
    @echo "  just \033[0;33mtest\033[0m                    \033[0;32mRun Rust tests with nextest\033[0m"
    @echo ""
    @echo "\033[1;35m  Utilities:\033[0m"
    @echo "  just \033[0;33mfmt\033[0m                     \033[0;32mFormat Rust source files\033[0m"
    @echo "  just \033[0;33minstall-tools\033[0m           \033[0;32mInstall clippy, rustfmt, sccache, and nextest\033[0m"
    @echo ""

# ============================================================================
# Development Commands
# ============================================================================

import 'justfiles/development/dev.just'
import 'justfiles/development/dev-stub.just'

# ============================================================================
# Building Commands
# ============================================================================

import 'justfiles/building/release.just'

# ============================================================================
# Verification Commands
# ============================================================================

import 'justfiles/verification/fmt-check.just'
import 'justfiles/verification/cargo-check.just'
import 'justfiles/verification/clippy.just'
import 'justfiles/verification/lint.just'
import 'justfiles/verification/check.just'

# ============================================================================
# Testing Commands
# ============================================================================

import 'justfiles/testing/test.just'

# ============================================================================
# Utilities Commands
# ============================================================================

import 'justfiles/utilities/fmt.just'
import 'justfiles/utilities/install-tools.just'
import 'justfiles/utilities/_require-nextest.just'
import 'justfiles/utilities/_require-sccache.just'
