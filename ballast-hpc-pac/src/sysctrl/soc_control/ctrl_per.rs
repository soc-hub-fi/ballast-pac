#[doc = "Register `CTRL_PER` reader"]
pub struct R(crate::R<CTRL_PER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_PER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_PER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_PER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_PER` writer"]
pub struct W(crate::W<CTRL_PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_PER_SPEC>;
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
impl From<crate::W<CTRL_PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Sel_hyper_axi` reader - "]
pub type SEL_HYPER_AXI_R = crate::BitReader<bool>;
#[doc = "Field `Sel_hyper_axi` writer - "]
pub type SEL_HYPER_AXI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_PER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sel_hyper_axi(&self) -> SEL_HYPER_AXI_R {
        SEL_HYPER_AXI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sel_hyper_axi(&mut self) -> SEL_HYPER_AXI_W<0> {
        SEL_HYPER_AXI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_per](index.html) module"]
pub struct CTRL_PER_SPEC;
impl crate::RegisterSpec for CTRL_PER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_per::R](R) reader structure"]
impl crate::Readable for CTRL_PER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_per::W](W) writer structure"]
impl crate::Writable for CTRL_PER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_PER to value 0"]
impl crate::Resettable for CTRL_PER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
