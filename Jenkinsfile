#!groovy

// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

pipeline {
    agent {
        node {
            label 'master'
            customWorkspace "workspace/${env.BUILD_TAG}"
        }
    }

    triggers {
        cron(env.BRANCH_NAME == 'master' ? 'H 3 * * *' : '')
    }

    options {
        timestamps()
        buildDiscarder(logRotator(daysToKeepStr: '31'))
    }

    environment {
        ISOLATION_ID = sh(returnStdout: true, script: 'printf $BUILD_TAG | sha256sum | cut -c1-64').trim()
        COMPOSE_PROJECT_NAME = sh(returnStdout: true, script: 'printf $BUILD_TAG | sha256sum | cut -c1-64').trim()
    }

    stages {
        stage('Check Whitelist') {
            steps {
                readTrusted 'bin/whitelist'
                sh './bin/whitelist "$CHANGE_AUTHOR" /etc/jenkins-authorized-builders'
            }
            when {
                not {
                    branch 'master'
                }
            }
        }

        stage('Check for Signed-Off Commits') {
            steps {
                sh '''#!/bin/bash -l
                    if [ -v CHANGE_URL ] ;
                    then
                        temp_url="$(echo $CHANGE_URL |sed s#github.com/#api.github.com/repos/#)/commits"
                        pull_url="$(echo $temp_url |sed s#pull#pulls#)"

                        IFS=$'\n'
                        for m in $(curl -s "$pull_url" | grep "message") ; do
                            if echo "$m" | grep -qi signed-off-by:
                            then
                              continue
                            else
                              echo "FAIL: Missing Signed-Off Field"
                              echo "$m"
                              exit 1
                            fi
                        done
                        unset IFS;
                    fi
                '''
            }
        }

        stage('Fetch Tags') {
            steps {
                sh 'git fetch --tag'
            }
        }

        stage('Run Lint') {
            steps {
              sh 'docker build . -f docker/lint -t lint-smart:$ISOLATION_ID'
              sh 'docker run --rm -v $(pwd):/project/sawtooth-smart lint-smart:$ISOLATION_ID'
            }
        }

        stage('Build Smart') {
            steps {
                sh 'docker-compose -f docker-compose-installed.yaml build dgc-contract-cli'
                sh 'docker-compose -f docker-compose-installed.yaml build dgc-contract-tp'
                sh 'docker-compose -f docker-compose-installed.yaml build intkey_multiply'

            }
        }

        stage('Test Smart') {
            steps {
                sh 'docker-compose -f docker-compose.yaml -f integration/smart_test.yaml up --build --abort-on-container-exit --exit-code-from test_smart'
            }
        }

        stage('Create Git Archive') {
            steps {
                sh '''
                    REPO=$(git remote show -n origin | grep Fetch | awk -F'[/.]' '{print $6}')
                    VERSION=`git describe --dirty`
                    git archive HEAD --format=zip -9 --output=$REPO-$VERSION.zip
                    git archive HEAD --format=tgz -9 --output=$REPO-$VERSION.tgz
                '''
            }
        }

        stage ('Build Documentation') {
            steps {
                sh 'docker-compose -f docs/docker-compose.yaml up'
                sh 'docker-compose -f docs/docker-compose.yaml down'
            }
        }

        stage('Build Archive Artifacts') {
            steps {
                sh 'mkdir -p build/debs'
                sh 'docker run --rm -v $(pwd)/build/debs:/build/debs --entrypoint "/bin/bash" sawtooth-dgc-contract-cli:${ISOLATION_ID} "-c" "cp /tmp/*.deb /build/debs"'
                sh 'docker run --rm -v $(pwd)/build/debs:/build/debs --entrypoint "/bin/bash" sawtooth-dgc-contract-tp:${ISOLATION_ID} "-c" "cp /tmp/*.deb /build/debs"'
                sh 'docker run --rm -v $(pwd)/build/scar:/build/scar --entrypoint "/bin/bash" intkeym-scar:${ISOLATION_ID} "-c" "cp /tmp/*.scar /build/scar"'
            }
        }
    }
    post {
        always {
            sh 'docker-compose -f docker-compose.yaml -f integration/smart_test.yaml down'
            sh 'docker-compose -f docs/docker-compose.yaml down'
        }
        success {
            archiveArtifacts artifacts: '*.tgz, *.zip, build/debs/*.deb, build/scar/*.scar, docs/build/html/**'
        }
        aborted {
            error "Aborted, exiting now"
        }
        failure {
            error "Failed, exiting now"
        }
    }
}
