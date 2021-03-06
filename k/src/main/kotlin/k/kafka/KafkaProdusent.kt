package k.kafka

import k.Environment
import org.apache.kafka.clients.CommonClientConfigs
import org.apache.kafka.clients.producer.KafkaProducer
import org.apache.kafka.clients.producer.ProducerConfig
import org.apache.kafka.common.config.SaslConfigs
import org.apache.kafka.common.serialization.StringSerializer

private fun commonConfig(env: Environment): Map<String, String> {
    return mapOf(
        CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG to env.brokers,
        CommonClientConfigs.SECURITY_PROTOCOL_CONFIG to env.kafkaSecurityProtocol,
        SaslConfigs.SASL_MECHANISM to "PLAIN"
    )
}

private fun commonProducerConfig(
    env: Environment,
    keySerializer: Class<*>,
    valueSerializer: Class<*>
): Map<String, Any> {
    return mapOf(
        ProducerConfig.ACKS_CONFIG to "all",
        ProducerConfig.ENABLE_IDEMPOTENCE_CONFIG to "true",
        ProducerConfig.MAX_IN_FLIGHT_REQUESTS_PER_CONNECTION to "1",
        ProducerConfig.MAX_BLOCK_MS_CONFIG to "15000",
        ProducerConfig.RETRIES_CONFIG to "100000",
        ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG to valueSerializer,
        ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG to keySerializer
    ) + commonConfig(env)
}

fun <K, V> createProducer(env: Environment): KafkaProducer<K, V> =
    KafkaProducer(
        commonProducerConfig(
            env = env,
            keySerializer = StringSerializer::class.java,
            valueSerializer = StringSerializer::class.java
        )
    )
