package testconfiguration.models

data class ProvenanceTxEvents(val events: List<ProvenanceTxEvent>)

data class ProvenanceTxEvent(val type: String, val attributes: List<ProvenanceTxEventAttribute>)

data class ProvenanceTxEventAttribute(val key: String?, val value: String?)
