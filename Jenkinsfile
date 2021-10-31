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
        sh 'cargo build'
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

        stage('clippy') {
          steps {
            unstash 'cargo-build'
            sh 'rustup component add clippy'
            sh 'cargo clippy -- -D warnings'
          }
        }

      }
    }

    stage('build-image') {
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