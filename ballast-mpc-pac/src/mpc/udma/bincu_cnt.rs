#[doc = "Register `BINCU_CNT` reader"]
pub struct R(crate::R<BINCU_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BINCU_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BINCU_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BINCU_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BINCU_CNT` writer"]
pub struct W(crate::W<BINCU_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BINCU_CNT_SPEC>;
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
impl From<crate::W<BINCU_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BINCU_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Binarization and counting unit count value set."]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Binarization and counting unit count value set."]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BINCU_CNT_SPEC, u32, u32, 20, O>;
#[doc = "Field `EN` reader - Binarization and counting unit enable: -1'b0: not enable -1'b1: enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Binarization and counting unit enable: -1'b0: not enable -1'b1: enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BINCU_CNT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - Binarization and counting unit count value set."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 31 - Binarization and counting unit enable: -1'b0: not enable -1'b1: enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Binarization and counting unit count value set."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Bit 31 - Binarization and counting unit enable: -1'b0: not enable -1'b1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FILTER binarization count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bincu_cnt](index.html) module"]
pub struct BINCU_CNT_SPEC;
impl crate::RegisterSpec for BINCU_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bincu_cnt::R](R) reader structure"]
impl crate::Readable for BINCU_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bincu_cnt::W](W) writer structure"]
impl crate::Writable for BINCU_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BINCU_CNT to value 0"]
impl crate::Resettable for BINCU_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
