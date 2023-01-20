#[doc = "Register `Speed` reader"]
pub struct R(crate::R<SPEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Speed` writer"]
pub struct W(crate::W<SPEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPEED_SPEC>;
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
impl From<crate::W<SPEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Speed` reader - This register is used to set speed level of ethernet mac core."]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Speed` writer - This register is used to set speed level of ethernet mac core."]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPEED_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - This register is used to set speed level of ethernet mac core."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register is used to set speed level of ethernet mac core."]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<0> {
        SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set speed level of ethernet mac core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [speed](index.html) module"]
pub struct SPEED_SPEC;
impl crate::RegisterSpec for SPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [speed::R](R) reader structure"]
impl crate::Readable for SPEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [speed::W](W) writer structure"]
impl crate::Writable for SPEED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Speed to value 0x04"]
impl crate::Resettable for SPEED_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
