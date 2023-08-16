package testconfiguration.util

import kotlinx.coroutines.delay
import mu.KotlinLogging
import testconfiguration.extensions.KLogLevel
import testconfiguration.extensions.logDynamic

object CoroutineUtil {
    private val logger = KotlinLogging.logger { }

    suspend fun <T> withRetryBackoff(
        errorPrefix: String,
        times: Int = 10,
        initialDelay: Long = 100L,
        maxDelay: Long = 1000L,
        factor: Double = 2.0,
        showStackTraceInFailures: Boolean = true,
        block: suspend () -> T,
    ): T {
        var currentDelay = initialDelay
        repeat(times - 1) { attempt ->
            try {
                return block()
            } catch (e: Exception) {
                logger.logDynamic(
                    level = if (attempt >= times / 2) KLogLevel.ERROR else KLogLevel.WARN,
                    message = "$errorPrefix: Retry block failed for attempt [${attempt + 1} / $times]. Waiting for [${currentDelay}ms]",
                    t = e.takeIf { showStackTraceInFailures },
                )
            }
            delay(currentDelay)
            currentDelay = (currentDelay * factor).toLong().coerceAtMost(maxDelay)
        }
        // If all else fails, attempt the block once more
        return block()
    }
}
