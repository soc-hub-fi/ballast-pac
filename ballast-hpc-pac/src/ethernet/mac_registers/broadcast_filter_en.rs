#[doc = "Register `broadcast_filter_en` reader"]
pub struct R(crate::R<BROADCAST_FILTER_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROADCAST_FILTER_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROADCAST_FILTER_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROADCAST_FILTER_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `broadcast_filter_en` writer"]
pub struct W(crate::W<BROADCAST_FILTER_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROADCAST_FILTER_EN_SPEC>;
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
impl From<crate::W<BROADCAST_FILTER_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROADCAST_FILTER_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `broadcast_filter_en` reader - The broadcast packet filter will enable only when broadcast_filter_en is set"]
pub type BROADCAST_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `broadcast_filter_en` writer - The broadcast packet filter will enable only when broadcast_filter_en is set"]
pub type BROADCAST_FILTER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BROADCAST_FILTER_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The broadcast packet filter will enable only when broadcast_filter_en is set"]
    #[inline(always)]
    pub fn broadcast_filter_en(&self) -> BROADCAST_FILTER_EN_R {
        BROADCAST_FILTER_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The broadcast packet filter will enable only when broadcast_filter_en is set"]
    #[inline(always)]
    #[must_use]
    pub fn broadcast_filter_en(&mut self) -> BROADCAST_FILTER_EN_W<0> {
        BROADCAST_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The broadcast packet filter will enable only when broadcast_filter_en is set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [broadcast_filter_en](index.html) module"]
pub struct BROADCAST_FILTER_EN_SPEC;
impl crate::RegisterSpec for BROADCAST_FILTER_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [broadcast_filter_en::R](R) reader structure"]
impl crate::Readable for BROADCAST_FILTER_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [broadcast_filter_en::W](W) writer structure"]
impl crate::Writable for BROADCAST_FILTER_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets broadcast_filter_en to value 0"]
impl crate::Resettable for BROADCAST_FILTER_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
