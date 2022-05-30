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
pub struct PARITY_ENA_R(crate::FieldReader<bool>);
impl PARITY_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ENA` writer - Parity bit generation and check configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub struct PARITY_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `BIT_LENGTH` reader - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
pub struct BIT_LENGTH_R(crate::FieldReader<u8>);
impl BIT_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIT_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_LENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_LENGTH` writer - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
pub struct BIT_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u32 & 3) << 1);
        self.w
    }
}
#[doc = "Field `STOP_BITS` reader - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
pub struct STOP_BITS_R(crate::FieldReader<bool>);
impl STOP_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_BITS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_BITS` writer - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
pub struct STOP_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BITS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `POLLING_EN` reader - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
pub struct POLLING_EN_R(crate::FieldReader<bool>);
impl POLLING_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLLING_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLLING_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLLING_EN` writer - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
pub struct POLLING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLING_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `CLEAN_FIFO` reader - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
pub struct CLEAN_FIFO_R(crate::FieldReader<bool>);
impl CLEAN_FIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLEAN_FIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAN_FIFO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAN_FIFO` writer - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
pub struct CLEAN_FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAN_FIFO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `TX_ENA` reader - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub struct TX_ENA_R(crate::FieldReader<bool>);
impl TX_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ENA` writer - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub struct TX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RX_ENA` reader - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub struct RX_ENA_R(crate::FieldReader<bool>);
impl RX_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ENA` writer - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
pub struct RX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
pub struct CLKDIV_R(crate::FieldReader<u16>);
impl CLKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
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
    pub fn parity_ena(&mut self) -> PARITY_ENA_W {
        PARITY_ENA_W { w: self }
    }
    #[doc = "Bits 1:2 - Character length bitfield: - 2'b00: 5 bits - 2'b01: 6 bits - 2'b10: 7 bits - 2'b11: 8 bits"]
    #[inline(always)]
    pub fn bit_length(&mut self) -> BIT_LENGTH_W {
        BIT_LENGTH_W { w: self }
    }
    #[doc = "Bit 3 - Stop bits length bitfield: - 1'b0: 1 stop bit - 1'b1: 2 stop bits"]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W {
        STOP_BITS_W { w: self }
    }
    #[doc = "Bit 4 - When in uart read, use polling method to read the data, read interrupt enable flag will be ignored: - 1'b0: Do not using polling method to read data. - 1'b1: Using polling method to read data. Interrupt enable flag will be ignored."]
    #[inline(always)]
    pub fn polling_en(&mut self) -> POLLING_EN_W {
        POLLING_EN_W { w: self }
    }
    #[doc = "Bit 5 - In all mode clean the RX fifo, set 1 then set 0 to realize a reset fifo: - 1'b0: Stop Clean the RX FIFO. - 1'b1: Clean the RX FIFO."]
    #[inline(always)]
    pub fn clean_fifo(&mut self) -> CLEAN_FIFO_W {
        CLEAN_FIFO_W { w: self }
    }
    #[doc = "Bit 8 - TX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn tx_ena(&mut self) -> TX_ENA_W {
        TX_ENA_W { w: self }
    }
    #[doc = "Bit 9 - RX transceiver configuration bitfield: - 1'b0: disabled - 1'b1: enabled"]
    #[inline(always)]
    pub fn rx_ena(&mut self) -> RX_ENA_W {
        RX_ENA_W { w: self }
    }
    #[doc = "Bits 16:31 - UART Clock divider configuration bitfield. The baudrate is equal to SOC_FREQ/CLKDIV."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
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
}
#[doc = "`reset()` method sets UART_SETUP to value 0"]
impl crate::Resettable for UART_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
