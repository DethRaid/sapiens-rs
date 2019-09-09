pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                powershell "cargo build"
            }
        }
    }
}