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
pub struct NO_PREAMPLE_R(crate::FieldReader<bool, bool>);
impl NO_PREAMPLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NO_PREAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_PREAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `no_preample` writer - Bit 0 - No Preamble: When this bit is unset, read or write operations start with a 32-bit preamble. When it is set, preamble is not used."]
pub struct NO_PREAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_PREAMPLE_W<'a> {
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
#[doc = "Field `start_write` reader - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
pub struct START_WRITE_R(crate::FieldReader<bool, bool>);
impl START_WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `start_write` writer - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
pub struct START_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_WRITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `start_read` reader - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
pub struct START_READ_R(crate::FieldReader<bool, bool>);
impl START_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `start_read` writer - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
pub struct START_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> START_READ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
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
    pub fn no_preample(&mut self) -> NO_PREAMPLE_W {
        NO_PREAMPLE_W { w: self }
    }
    #[doc = "Bit 1 - Bit 1 - Start Write: When this bit is set, MDIO logic starts a write operation with above write data, PHY and register addresses. This bit is automatically unset when the operation starts."]
    #[inline(always)]
    pub fn start_write(&mut self) -> START_WRITE_W {
        START_WRITE_W { w: self }
    }
    #[doc = "Bit 2 - Bit 2 - Start Read: When this bit is set, MDIO logic starts a read operations with above PHY and register addresses. This bit is automatically unset when the operation starts."]
    #[inline(always)]
    pub fn start_read(&mut self) -> START_READ_W {
        START_READ_W { w: self }
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
}
#[doc = "`reset()` method sets MDIO_Ctrl to value 0"]
impl crate::Resettable for MDIO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
