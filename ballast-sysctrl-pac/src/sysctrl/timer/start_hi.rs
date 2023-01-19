#[doc = "Register `START_HI` reader"]
pub struct R(crate::R<START_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `START_HI` writer"]
pub struct W(crate::W<START_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_HI_SPEC>;
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
impl From<crate::W<START_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_HI` reader - Timer high reset command (writes RST in CFG_LO)"]
pub type START_HI_R = crate::BitReader<bool>;
#[doc = "Field `START_HI` writer - Timer high reset command (writes RST in CFG_LO)"]
pub type START_HI_W<'a, const O: u8> = crate::BitWriter<'a, u32, START_HI_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer high reset command (writes RST in CFG_LO)"]
    #[inline(always)]
    pub fn start_hi(&self) -> START_HI_R {
        START_HI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer high reset command (writes RST in CFG_LO)"]
    #[inline(always)]
    #[must_use]
    pub fn start_hi(&mut self) -> START_HI_W<0> {
        START_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start Timer High counting register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start_hi](index.html) module"]
pub struct START_HI_SPEC;
impl crate::RegisterSpec for START_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start_hi::R](R) reader structure"]
impl crate::Readable for START_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [start_hi::W](W) writer structure"]
impl crate::Writable for START_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets START_HI to value 0"]
impl crate::Resettable for START_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
