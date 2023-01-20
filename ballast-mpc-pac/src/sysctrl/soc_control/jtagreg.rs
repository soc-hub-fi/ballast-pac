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
pub type JTAG_REG_WRITE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `jtag_reg_write` writer - "]
pub type JTAG_REG_WRITE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, JTAGREG_SPEC, u8, u8, 8, O>;
#[doc = "Field `jtag_reg_read` reader - "]
pub type JTAG_REG_READ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `jtag_reg_read` writer - "]
pub type JTAG_REG_READ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAGREG_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn jtag_reg_write(&mut self) -> JTAG_REG_WRITE_W<0> {
        JTAG_REG_WRITE_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_reg_read(&mut self) -> JTAG_REG_READ_W<8> {
        JTAG_REG_READ_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JTAGREG to value 0"]
impl crate::Resettable for JTAGREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
