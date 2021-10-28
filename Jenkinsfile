pipeline {
  agent {
    docker {
      image 'rust:latest'
    }

  }
  stages {
    stage('lint') {
      steps {
        sh 'rustup --version'
        sh 'rustc --version'
        sh 'cargo --version'
        sh 'rustup component add rustfmt'
        sh 'cargo fmt -- --check'
      }
    }

    stage('build') {
      steps {
        sh 'cargo build --verbose'
        stash(name: 'cargo-build', includes: 'target/*')
      }
    }

    stage('test') {
      parallel {
        stage('test-code') {
          steps {
            unstash 'cargo-build'
            sh 'cargo test --verbose'
          }
        }

        stage('audit-code') {
          steps {
            unstash 'cargo-build'
            sh 'cargo install cargo-audit'
            sh 'cargo audit'
            warnError(message: 'cargo audit failed')
          }
        }

        stage('clippy') {
          steps {
            unstash 'cargo-build'
            sh 'rustup component add clippy'
            sh 'cargo clippy -- -D warnings'
          }
        }

      }
    }

  }
  post {
    always {
      zulipNotification(stream: 'CI/CD')
    }

  }
  options {
    ansiColor('xterm')
  }
}