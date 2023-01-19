#[doc = "Register `T0_THRESHOLD` reader"]
pub struct R(crate::R<T0_THRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0_THRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0_THRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0_THRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0_THRESHOLD` writer"]
pub struct W(crate::W<T0_THRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0_THRESHOLD_SPEC>;
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
impl From<crate::W<T0_THRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0_THRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TH_LO` reader - ADV_TIMER0 threshold low part configuration bitfield. It defines start counter value"]
pub type TH_LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH_LO` writer - ADV_TIMER0 threshold low part configuration bitfield. It defines start counter value"]
pub type TH_LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_THRESHOLD_SPEC, u16, u16, 16, O>;
#[doc = "Field `TH_HI` reader - ADV_TIMER0 threshold high part configuration bitfield. It defines end counter value."]
pub type TH_HI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TH_HI` writer - ADV_TIMER0 threshold high part configuration bitfield. It defines end counter value."]
pub type TH_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_THRESHOLD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ADV_TIMER0 threshold low part configuration bitfield. It defines start counter value"]
    #[inline(always)]
    pub fn th_lo(&self) -> TH_LO_R {
        TH_LO_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADV_TIMER0 threshold high part configuration bitfield. It defines end counter value."]
    #[inline(always)]
    pub fn th_hi(&self) -> TH_HI_R {
        TH_HI_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADV_TIMER0 threshold low part configuration bitfield. It defines start counter value"]
    #[inline(always)]
    #[must_use]
    pub fn th_lo(&mut self) -> TH_LO_W<0> {
        TH_LO_W::new(self)
    }
    #[doc = "Bits 16:31 - ADV_TIMER0 threshold high part configuration bitfield. It defines end counter value."]
    #[inline(always)]
    #[must_use]
    pub fn th_hi(&mut self) -> TH_HI_W<16> {
        TH_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV_TIMER0 threshold configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0_threshold](index.html) module"]
pub struct T0_THRESHOLD_SPEC;
impl crate::RegisterSpec for T0_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0_threshold::R](R) reader structure"]
impl crate::Readable for T0_THRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0_threshold::W](W) writer structure"]
impl crate::Writable for T0_THRESHOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0_THRESHOLD to value 0"]
impl crate::Resettable for T0_THRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
