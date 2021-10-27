pipeline {
  agent {
    docker {
      image 'rust:latest'
    }

  }
  stages {
    stage('lint-code') {
      steps {
        sh 'rustup --version'
        sh 'rustc --version'
        sh 'cargo --version'
        sh 'rustup component add rustfmt'
        sh 'cargo fmt -- --check'
        sh 'rustup component add clippy'
        sh 'cargo clippy -- -D warnings'
      }
    }

    stage('build') {
      steps {
        sh 'cargo build --verbose'
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