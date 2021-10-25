pipeline {
  agent {
    docker {
      image 'rust:latest'
    }

  }
  stages {
    stage('test') {
      parallel {
        stage('test-code') {
          agent {
            docker {
              image 'rust:latest'
            }

          }
          steps {
            sh 'cargo test'
            sh 'cargo install cargo-tarpaulin'
            sh 'cargo tarpaulin --ignore-tests'
          }
        }

        stage('lint-code') {
          agent {
            docker {
              image 'rust:latest'
            }

          }
          steps {
            sh 'rustup component add rustfmt'
            sh 'cargo fmt -- --check'
            sh 'rustup component add clippy'
            sh 'cargo clippy -- -D warnings'
          }
        }

        stage('audit-code') {
          agent {
            docker {
              image 'rust:latest'
            }

          }
          steps {
            sh 'cargo install cargo-audit'
            sh 'cargo audit'
          }
        }

      }
    }

  }
}