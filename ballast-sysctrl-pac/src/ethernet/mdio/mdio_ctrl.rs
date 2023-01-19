#[doc = "Register `MDIO_Ctrl` reader"]
pub struct R(crate::R<MDIO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIO_Ctrl` writer"]
pub struct W(crate::W<MDIO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIO_CTRL_SPEC>;
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
impl From<crate::W<MDIO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `no_preample` reader - Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used."]
pub type NO_PREAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `no_preample` writer - Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used."]
pub type NO_PREAMPLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDIO_CTRL_SPEC, bool, O>;
#[doc = "Field `start_write` reader - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
pub type START_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `start_write` writer - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
pub type START_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDIO_CTRL_SPEC, bool, O>;
#[doc = "Field `start_read` reader - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
pub type START_READ_R = crate::BitReader<bool>;
#[doc = "Field `start_read` writer - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
pub type START_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDIO_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used."]
    #[inline(always)]
    pub fn no_preample(&self) -> NO_PREAMPLE_R {
        NO_PREAMPLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
    #[inline(always)]
    pub fn start_write(&self) -> START_WRITE_R {
        START_WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
    #[inline(always)]
    pub fn start_read(&self) -> START_READ_R {
        START_READ_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used."]
    #[inline(always)]
    #[must_use]
    pub fn no_preample(&mut self) -> NO_PREAMPLE_W<0> {
        NO_PREAMPLE_W::new(self)
    }
    #[doc = "Bit 1 - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
    #[inline(always)]
    #[must_use]
    pub fn start_write(&mut self) -> START_WRITE_W<1> {
        START_WRITE_W::new(self)
    }
    #[doc = "Bit 2 - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
    #[inline(always)]
    #[must_use]
    pub fn start_read(&mut self) -> START_READ_W<2> {
        START_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Three-bit bitfield. Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used. Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts. Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts. Bits 1 and 2 should not be set simultaneously.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_ctrl](index.html) module"]
pub struct MDIO_CTRL_SPEC;
impl crate::RegisterSpec for MDIO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_ctrl::R](R) reader structure"]
impl crate::Readable for MDIO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdio_ctrl::W](W) writer structure"]
impl crate::Writable for MDIO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIO_Ctrl to value 0"]
impl crate::Resettable for MDIO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
