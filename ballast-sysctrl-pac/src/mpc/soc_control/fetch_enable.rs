#[doc = "Register `FETCH_ENABLE` reader"]
pub struct R(crate::R<FETCH_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCH_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCH_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCH_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCH_ENABLE` writer"]
pub struct W(crate::W<FETCH_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCH_ENABLE_SPEC>;
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
impl From<crate::W<FETCH_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCH_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `E` reader - Enable Fetch"]
pub type E_R = crate::BitReader<bool>;
#[doc = "Field `E` writer - Enable Fetch"]
pub type E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FETCH_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Fetch"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Fetch"]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> E_W<0> {
        E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the value of the fetch enable signal of the core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetch_enable](index.html) module"]
pub struct FETCH_ENABLE_SPEC;
impl crate::RegisterSpec for FETCH_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetch_enable::R](R) reader structure"]
impl crate::Readable for FETCH_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetch_enable::W](W) writer structure"]
impl crate::Writable for FETCH_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FETCH_ENABLE to value 0x01"]
impl crate::Resettable for FETCH_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
