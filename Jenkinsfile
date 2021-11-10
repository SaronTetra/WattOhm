pipeline {
  agent any
  stages {
    stage('lint') {
      agent {
        docker {
          image 'rust:latest'
        }

      }
      steps {
        sh 'rustup --version'
        sh 'rustc --version'
        sh 'cargo --version'
        sh 'rustup component add rustfmt'
        sh 'cargo fmt -- --check'
      }
    }

    stage('build') {
      agent {
        docker {
          image 'rust:latest'
        }

      }
      when {
        anyOf {
          changeRequest target: 'master'
          branch 'master'
        }

      }
      steps {
        sh 'cargo build'
        stash(name: 'cargo-build', includes: 'target/**/*')
      }
    }

    stage('test') {
      when {
        anyOf {
          changeRequest target: 'master'
          branch 'master'
        }

      }
      parallel {
        stage('test-code') {
          agent {
            docker {
              image 'rust:latest'
            }

          }
          steps {
            unstash 'cargo-build'
            sh 'cargo test'
          }
        }

        stage('clippy') {
          agent {
            docker {
              image 'rust:latest'
            }

          }
          steps {
            unstash 'cargo-build'
            sh 'rustup component add clippy'
            sh 'cargo clippy -- -D warnings'
          }
        }

      }
    }

    stage('build-image') {
      when {
        branch 'master'
      }
      steps {
        withCredentials(bindings: [usernamePassword(credentialsId: 'dockerHub', usernameVariable: 'HUB_USER', passwordVariable: 'HUB_TOKEN')]) {
          sh '''
                        docker login -u $HUB_USER -p $HUB_TOKEN 
                        docker build -t $HUB_USER/wattohm .
                        docker image push $HUB_USER/wattohm
                    '''
        }

      }
    }

    stage('deploy') {
      when {
        branch 'master'
      }
      steps {
        withCredentials(bindings: [string(credentialsId: 'vpsIP', variable: 'VPS_IP')]) {
          sh '''
              ssh deploy@$VPS_IP "cd ~/docker/wattohm &&
              docker-compose pull &&
              docker-compose up --force-recreate -d"
          '''
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