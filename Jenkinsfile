pipeline {
  agent {
    docker {
      image 'rust:latest'
    }

  }
  stages {
    stage('build') {
      parallel {
        stage('version') {
          steps {
            sh 'cargo --version'
            sh 'rustup --version'
            sh 'rustc --version'
          }
        }

        stage('build') {
          steps {
            sh 'cargo build --verbose'
          }
        }

      }
    }

    stage('test') {
      parallel {
        stage('test-code') {
          steps {
            sh 'cargo test --verbose'
            sh 'cargo install cargo-tarpaulin'
            sh 'cargo tarpaulin --ignore-tests'
          }
        }

        stage('lint-code') {
          steps {
            sh 'rustup component add rustfmt'
            sh 'cargo fmt -- --check'
            sh 'rustup component add clippy'
            sh 'cargo clippy -- -D warnings'
          }
        }

        stage('audit-code') {
          steps {
            sh 'cargo install cargo-audit'
            sh 'cargo audit'
          }
        }

      }
    }

  }
}