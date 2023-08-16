package io.provenance.bilateral.extensions

import com.google.protobuf.ByteString

/**
 * A simple extension to convert a ByteString to its raw string content in UTF_8 encoding.  Not all byte strings will
 * include sane values when this function is used on them, but some byte strings do represent raw strings and are
 * encoded this way simply due to protobuf requirements.
 */
internal fun ByteString.toStringUTF8(): String = this.toString(Charsets.UTF_8)
