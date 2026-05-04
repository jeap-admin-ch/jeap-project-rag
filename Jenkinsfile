@Library('jeap-microservice-pipeline@v2') _

jeapBuildPipeline(
    mavenImage: 'bit/eclipse-temurin:25',
    mavenDockerUser: 'jenkins',
    buildMavenDockerArgs: '-v /var/run/docker.sock:/var/run/docker.sock',
    branch: [
        MASTER : [
            systemIntegrationTest: false,
            nextStage			 : null,
            integrationTest      : true,
            qualityCheck         : true,
            qualityGateCheck     : false,
            publish              : false, // TODO
            deployStage          : null,
            buildNumberGenerator: ch.admin.bit.jeap.microservicePipeline.branching.BuildNumberGenerator.TIMESTAMP
        ],
        FEATURE: [
            integrationTest      : true,
            qualityCheck         : true,
            qualityGateCheck     : false,
            publish              : true,
            deployStage          : null,
            systemIntegrationTest: false,
            buildNumberGenerator: ch.admin.bit.jeap.microservicePipeline.branching.BuildNumberGenerator.BRANCH_NAME_SNAPSHOT
        ]
    ]
)

