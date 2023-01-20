#[doc = "Register `D_PRA_CFG` reader"]
pub struct R(crate::R<D_PRA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PRA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PRA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PRA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_PRA_CFG` writer"]
pub struct W(crate::W<D_PRA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_PRA_CFG_SPEC>;
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
impl From<crate::W<D_PRA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_PRA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRA_TRUNCATE` reader - "]
pub type PRA_TRUNCATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRA_TRUNCATE` writer - "]
pub type PRA_TRUNCATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_PRA_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pra_truncate(&self) -> PRA_TRUNCATE_R {
        PRA_TRUNCATE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pra_truncate(&mut self) -> PRA_TRUNCATE_W<0> {
        PRA_TRUNCATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRA truncate in Winograd mode, range: 0~2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pra_cfg](index.html) module"]
pub struct D_PRA_CFG_SPEC;
impl crate::RegisterSpec for D_PRA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pra_cfg::R](R) reader structure"]
impl crate::Readable for D_PRA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_pra_cfg::W](W) writer structure"]
impl crate::Writable for D_PRA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_PRA_CFG to value 0"]
impl crate::Resettable for D_PRA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
