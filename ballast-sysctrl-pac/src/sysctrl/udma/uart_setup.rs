#[doc = "Register `UART_SETUP` reader"]
pub struct R(crate::R<UART_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SETUP` writer"]
pub struct W(crate::W<UART_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UART_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARITY_ENA` reader - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type PARITY_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENA` writer - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type PARITY_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_SETUP_SPEC, bool, O>;
#[doc = "Field `BIT_LENGTH` reader - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
pub type BIT_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIT_LENGTH` writer - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
pub type BIT_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_SETUP_SPEC, u8, u8, 2, O>;
#[doc = "Field `STOP_BITS` reader - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
pub type STOP_BITS_R = crate::BitReader<bool>;
#[doc = "Field `STOP_BITS` writer - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
pub type STOP_BITS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_SETUP_SPEC, bool, O>;
#[doc = "Field `POLLING_EN` reader - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
pub type POLLING_EN_R = crate::BitReader<bool>;
#[doc = "Field `POLLING_EN` writer - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
pub type POLLING_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_SETUP_SPEC, bool, O>;
#[doc = "Field `CLEAN_FIFO` reader - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
pub type CLEAN_FIFO_R = crate::BitReader<bool>;
#[doc = "Field `CLEAN_FIFO` writer - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
pub type CLEAN_FIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_SETUP_SPEC, bool, O>;
#[doc = "Field `TX_ENA` reader - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type TX_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_ENA` writer - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type TX_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_SETUP_SPEC, bool, O>;
#[doc = "Field `RX_ENA` reader - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type RX_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_ENA` writer - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub type RX_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_SETUP_SPEC, bool, O>;
#[doc = "Field `CLKDIV` reader - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV` writer - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UART_SETUP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn parity_ena(&self) -> PARITY_ENA_R {
        PARITY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
    #[inline(always)]
    pub fn bit_length(&self) -> BIT_LENGTH_R {
        BIT_LENGTH_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
    #[inline(always)]
    pub fn polling_en(&self) -> POLLING_EN_R {
        POLLING_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
    #[inline(always)]
    pub fn clean_fifo(&self) -> CLEAN_FIFO_R {
        CLEAN_FIFO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn tx_ena(&self) -> TX_ENA_R {
        TX_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn rx_ena(&self) -> RX_ENA_R {
        RX_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn parity_ena(&mut self) -> PARITY_ENA_W<0> {
        PARITY_ENA_W::new(self)
    }
    #[doc = "Bits 1:2 - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn bit_length(&mut self) -> BIT_LENGTH_W<1> {
        BIT_LENGTH_W::new(self)
    }
    #[doc = "Bit 3 - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<3> {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 4 - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn polling_en(&mut self) -> POLLING_EN_W<4> {
        POLLING_EN_W::new(self)
    }
    #[doc = "Bit 5 - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn clean_fifo(&mut self) -> CLEAN_FIFO_W<5> {
        CLEAN_FIFO_W::new(self)
    }
    #[doc = "Bit 8 - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ena(&mut self) -> TX_ENA_W<8> {
        TX_ENA_W::new(self)
    }
    #[doc = "Bit 9 - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ena(&mut self) -> RX_ENA_W<9> {
        RX_ENA_W::new(self)
    }
    #[doc = "Bits 16:31 - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<16> {
        CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDMA UART configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_setup](index.html) module"]
pub struct UART_SETUP_SPEC;
impl crate::RegisterSpec for UART_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_setup::R](R) reader structure"]
impl crate::Readable for UART_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_setup::W](W) writer structure"]
impl crate::Writable for UART_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_SETUP to value 0"]
impl crate::Resettable for UART_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
