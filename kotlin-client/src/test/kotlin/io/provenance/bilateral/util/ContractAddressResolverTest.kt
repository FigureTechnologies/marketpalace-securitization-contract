package io.provenance.bilateral.util

import io.mockk.every
import io.mockk.mockk
import io.provenance.client.grpc.PbClient
import io.provenance.name.v1.QueryResolveResponse
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class ContractAddressResolverTest {
    @Test
    fun testResolveAddressPassThrough() {
        val address = "mockaddress"
        val resolver = ContractAddressResolver.ProvidedAddress(address)
        assertEquals(
            expected = address,
            actual = resolver.getAddress(mockk()),
            message = "Expected the resolver to pass through and just use the provided address instead of breaking on the mocked PbClient",
        )
    }

    @Test
    fun testResolveAddressWithNameLookup() {
        val name = "myname"
        val expectedAddress = "myaddress"
        val pbClient = mockk<PbClient>()
        every { pbClient.nameClient.resolve(any()) } returns QueryResolveResponse.newBuilder().setAddress(expectedAddress).build()
        val resolver = ContractAddressResolver.FromName(name)
        assertEquals(
            expected = expectedAddress,
            actual = resolver.getAddress(pbClient),
            message = "Expected the PbClient to be used when a name is provided",
        )
    }
}
