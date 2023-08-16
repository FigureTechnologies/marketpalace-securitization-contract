package testconfiguration.extensions

import mu.KLogger

enum class KLogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

fun KLogger.logDynamic(level: KLogLevel, message: String, t: Throwable? = null) = logDynamicInternal(level, message, t)

fun KLogger.logDynamic(level: KLogLevel, t: Throwable? = null, message: () -> String) = logDynamicInternal(level, message(), t)

private fun KLogger.logDynamicInternal(level: KLogLevel, message: String, throwable: Throwable?) {
    when (level) {
        KLogLevel.TRACE -> this.trace(message, throwable)
        KLogLevel.DEBUG -> this.debug(message, throwable)
        KLogLevel.INFO -> this.info(message, throwable)
        KLogLevel.WARN -> this.warn(message, throwable)
        KLogLevel.ERROR -> this.error(message, throwable)
    }
}
