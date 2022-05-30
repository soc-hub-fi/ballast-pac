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
pub struct PRA_TRUNCATE_R(crate::FieldReader<u8>);
impl PRA_TRUNCATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRA_TRUNCATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRA_TRUNCATE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRA_TRUNCATE` writer - "]
pub struct PRA_TRUNCATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRA_TRUNCATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
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
    pub fn pra_truncate(&mut self) -> PRA_TRUNCATE_W {
        PRA_TRUNCATE_W { w: self }
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
}
#[doc = "`reset()` method sets D_PRA_CFG to value 0"]
impl crate::Resettable for D_PRA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
