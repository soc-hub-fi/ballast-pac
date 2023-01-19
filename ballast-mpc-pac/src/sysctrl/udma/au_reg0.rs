#[doc = "Register `AU_REG0` reader"]
pub struct R(crate::R<AU_REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AU_REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AU_REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AU_REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AU_REG0` writer"]
pub struct W(crate::W<AU_REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AU_REG0_SPEC>;
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
impl From<crate::W<AU_REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AU_REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AU_REG0` reader - "]
pub type AU_REG0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AU_REG0` writer - "]
pub type AU_REG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AU_REG0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn au_reg0(&self) -> AU_REG0_R {
        AU_REG0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn au_reg0(&mut self) -> AU_REG0_W<0> {
        AU_REG0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER arithmetic unit 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [au_reg0](index.html) module"]
pub struct AU_REG0_SPEC;
impl crate::RegisterSpec for AU_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [au_reg0::R](R) reader structure"]
impl crate::Readable for AU_REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [au_reg0::W](W) writer structure"]
impl crate::Writable for AU_REG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AU_REG0 to value 0"]
impl crate::Resettable for AU_REG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
