#[doc = "Register `timer0_ctrl` reader"]
pub struct R(crate::R<TIMER0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer0_ctrl` writer"]
pub struct W(crate::W<TIMER0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CTRL_SPEC>;
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
impl From<crate::W<TIMER0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - "]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - "]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER0_CTRL_SPEC, bool, O>;
#[doc = "Field `prescaler` reader - "]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `prescaler` writer - "]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER0_CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<3> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_ctrl](index.html) module"]
pub struct TIMER0_CTRL_SPEC;
impl crate::RegisterSpec for TIMER0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_ctrl::R](R) reader structure"]
impl crate::Readable for TIMER0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_ctrl::W](W) writer structure"]
impl crate::Writable for TIMER0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets timer0_ctrl to value 0"]
impl crate::Resettable for TIMER0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
