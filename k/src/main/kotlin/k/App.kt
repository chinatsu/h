/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package k

import k.kafka.createProducer
import org.apache.kafka.clients.producer.ProducerRecord
import org.slf4j.Logger
import org.slf4j.LoggerFactory

val log: Logger = LoggerFactory.getLogger("k")

fun main(args: Array<String>) {
    val env = Environment()

    val producer = createProducer<String, String>(env)

    val start = System.currentTimeMillis()
    repeat(10_000) {
        try {
            producer.send(ProducerRecord(
                env.topic,
                "Key $it",
                "Message $it"
            )).get()
        } catch (e: Exception) {
            log.error("Something went wrong while producing message $it: ${e.message}")
            throw e
        }
    }
    log.info("Time elapsed in produce() is: ${(System.currentTimeMillis() - start)/1000.0}")
}
