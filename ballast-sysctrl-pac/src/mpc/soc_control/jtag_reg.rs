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
pub type JTAG_REG_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JTAG_REG_OUT` writer - "]
pub type JTAG_REG_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAG_REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `JTAG_REG_IN` reader - "]
pub type JTAG_REG_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JTAG_REG_IN` writer - "]
pub type JTAG_REG_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JTAG_REG_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn jtag_reg_out(&mut self) -> JTAG_REG_OUT_W<0> {
        JTAG_REG_OUT_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_reg_in(&mut self) -> JTAG_REG_IN_W<8> {
        JTAG_REG_IN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JTAG_REG to value 0"]
impl crate::Resettable for JTAG_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
