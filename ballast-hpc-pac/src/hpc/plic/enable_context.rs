#[doc = "Register `enable_context[%s]` reader"]
pub struct R(crate::R<ENABLE_CONTEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_CONTEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_CONTEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_CONTEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `enable_context[%s]` writer"]
pub struct W(crate::W<ENABLE_CONTEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_CONTEXT_SPEC>;
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
impl From<crate::W<ENABLE_CONTEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_CONTEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer0_int0_enable` reader - "]
pub type TIMER0_INT0_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `timer0_int0_enable` writer - "]
pub type TIMER0_INT0_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENABLE_CONTEXT_SPEC, bool, O>;
#[doc = "Field `timer0_int1_enable` reader - "]
pub type TIMER0_INT1_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `timer0_int1_enable` writer - "]
pub type TIMER0_INT1_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENABLE_CONTEXT_SPEC, bool, O>;
#[doc = "Field `timer1_int0_enable` reader - "]
pub type TIMER1_INT0_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `timer1_int0_enable` writer - "]
pub type TIMER1_INT0_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENABLE_CONTEXT_SPEC, bool, O>;
#[doc = "Field `timer1_int1_enable` reader - "]
pub type TIMER1_INT1_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `timer1_int1_enable` writer - "]
pub type TIMER1_INT1_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENABLE_CONTEXT_SPEC, bool, O>;
#[doc = "Field `external_int0_enable` reader - "]
pub type EXTERNAL_INT0_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `external_int0_enable` writer - "]
pub type EXTERNAL_INT0_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENABLE_CONTEXT_SPEC, bool, O>;
#[doc = "Field `external_int1_enable` reader - "]
pub type EXTERNAL_INT1_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `external_int1_enable` writer - "]
pub type EXTERNAL_INT1_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENABLE_CONTEXT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_int0_enable(&self) -> TIMER0_INT0_ENABLE_R {
        TIMER0_INT0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer0_int1_enable(&self) -> TIMER0_INT1_ENABLE_R {
        TIMER0_INT1_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer1_int0_enable(&self) -> TIMER1_INT0_ENABLE_R {
        TIMER1_INT0_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer1_int1_enable(&self) -> TIMER1_INT1_ENABLE_R {
        TIMER1_INT1_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn external_int0_enable(&self) -> EXTERNAL_INT0_ENABLE_R {
        EXTERNAL_INT0_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn external_int1_enable(&self) -> EXTERNAL_INT1_ENABLE_R {
        EXTERNAL_INT1_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_int0_enable(&mut self) -> TIMER0_INT0_ENABLE_W<0> {
        TIMER0_INT0_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_int1_enable(&mut self) -> TIMER0_INT1_ENABLE_W<1> {
        TIMER0_INT1_ENABLE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_int0_enable(&mut self) -> TIMER1_INT0_ENABLE_W<2> {
        TIMER1_INT0_ENABLE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_int1_enable(&mut self) -> TIMER1_INT1_ENABLE_W<3> {
        TIMER1_INT1_ENABLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn external_int0_enable(&mut self) -> EXTERNAL_INT0_ENABLE_W<4> {
        EXTERNAL_INT0_ENABLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn external_int1_enable(&mut self) -> EXTERNAL_INT1_ENABLE_W<5> {
        EXTERNAL_INT1_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_context](index.html) module"]
pub struct ENABLE_CONTEXT_SPEC;
impl crate::RegisterSpec for ENABLE_CONTEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable_context::R](R) reader structure"]
impl crate::Readable for ENABLE_CONTEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable_context::W](W) writer structure"]
impl crate::Writable for ENABLE_CONTEXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets enable_context[%s]
to value 0"]
impl crate::Resettable for ENABLE_CONTEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
