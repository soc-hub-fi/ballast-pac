#[doc = "Register `JTAGREG` reader"]
pub struct R(crate::R<JTAGREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAGREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAGREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAGREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAGREG` writer"]
pub struct W(crate::W<JTAGREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAGREG_SPEC>;
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
impl From<crate::W<JTAGREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAGREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `jtag_reg_write` reader - "]
pub struct JTAG_REG_WRITE_R(crate::FieldReader<u8, u8>);
impl JTAG_REG_WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JTAG_REG_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_REG_WRITE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `jtag_reg_write` writer - "]
pub struct JTAG_REG_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_REG_WRITE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `jtag_reg_read` reader - "]
pub struct JTAG_REG_READ_R(crate::FieldReader<u8, u8>);
impl JTAG_REG_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JTAG_REG_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_REG_READ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `jtag_reg_read` writer - "]
pub struct JTAG_REG_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_REG_READ_W<'a> {
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
    pub fn jtag_reg_write(&self) -> JTAG_REG_WRITE_R {
        JTAG_REG_WRITE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn jtag_reg_read(&self) -> JTAG_REG_READ_R {
        JTAG_REG_READ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn jtag_reg_write(&mut self) -> JTAG_REG_WRITE_W {
        JTAG_REG_WRITE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn jtag_reg_read(&mut self) -> JTAG_REG_READ_W {
        JTAG_REG_READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to read or write from JTAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagreg](index.html) module"]
pub struct JTAGREG_SPEC;
impl crate::RegisterSpec for JTAGREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtagreg::R](R) reader structure"]
impl crate::Readable for JTAGREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtagreg::W](W) writer structure"]
impl crate::Writable for JTAGREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JTAGREG to value 0"]
impl crate::Resettable for JTAGREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
