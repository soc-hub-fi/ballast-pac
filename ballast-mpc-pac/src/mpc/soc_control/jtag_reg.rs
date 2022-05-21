#[doc = "Register `JTAG_REG` reader"]
pub struct R(crate::R<JTAG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAG_REG` writer"]
pub struct W(crate::W<JTAG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAG_REG_SPEC>;
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
impl From<crate::W<JTAG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTAG_REG_OUT` reader - "]
pub struct JTAG_REG_OUT_R(crate::FieldReader<u8, u8>);
impl JTAG_REG_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JTAG_REG_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_REG_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_REG_OUT` writer - "]
pub struct JTAG_REG_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_REG_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `JTAG_REG_IN` reader - "]
pub struct JTAG_REG_IN_R(crate::FieldReader<u8, u8>);
impl JTAG_REG_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JTAG_REG_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_REG_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_REG_IN` writer - "]
pub struct JTAG_REG_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_REG_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn jtag_reg_out(&self) -> JTAG_REG_OUT_R {
        JTAG_REG_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn jtag_reg_in(&self) -> JTAG_REG_IN_R {
        JTAG_REG_IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn jtag_reg_out(&mut self) -> JTAG_REG_OUT_W {
        JTAG_REG_OUT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn jtag_reg_in(&mut self) -> JTAG_REG_IN_W {
        JTAG_REG_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the value of the input from the JTAG and can be used to write 8bit in the JTAG output register for system-to-JTAG communications.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtag_reg](index.html) module"]
pub struct JTAG_REG_SPEC;
impl crate::RegisterSpec for JTAG_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtag_reg::R](R) reader structure"]
impl crate::Readable for JTAG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtag_reg::W](W) writer structure"]
impl crate::Writable for JTAG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAG_REG to value 0"]
impl crate::Resettable for JTAG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
