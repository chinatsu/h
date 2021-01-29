package k

data class Environment (
    val brokers: String = getEnvVar("BROKERS", "localhost:29092"),
    val topic: String = getEnvVar("TOPIC", "h"),
    val kafkaSecurityProtocol: String = getEnvVar("KAFKA_SECURITY_PROTOCOL", "PLAINTEXT"),
    val messages: Int = getEnvVar("MESSAGES", "10000").toInt()
)


fun getEnvVar(varName: String, defaultValue: String? = null) =
    System.getenv(varName) ?: defaultValue ?: throw RuntimeException("Missing required variable \"$varName\"")
