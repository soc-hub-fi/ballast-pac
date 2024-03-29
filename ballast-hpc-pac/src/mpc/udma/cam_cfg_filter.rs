#[doc = "Register `CAM_CFG_FILTER` reader"]
pub struct R(crate::R<CAM_CFG_FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG_FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG_FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG_FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CFG_FILTER` writer"]
pub struct W(crate::W<CAM_CFG_FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG_FILTER_SPEC>;
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
impl From<crate::W<CAM_CFG_FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG_FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B_COEFF` reader - Coefficient that multiplies the B component NOTE: not used if FORMAT == BYPASS"]
pub type B_COEFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B_COEFF` writer - Coefficient that multiplies the B component NOTE: not used if FORMAT == BYPASS"]
pub type B_COEFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_FILTER_SPEC, u8, u8, 8, O>;
#[doc = "Field `G_COEFF` reader - Coefficient that multiplies the G component NOTE: not used if FORMAT == BYPASS"]
pub type G_COEFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G_COEFF` writer - Coefficient that multiplies the G component NOTE: not used if FORMAT == BYPASS"]
pub type G_COEFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_FILTER_SPEC, u8, u8, 8, O>;
#[doc = "Field `R_COEFF` reader - Coefficient that multiplies the R component NOTE: not used if FORMAT == BYPASS"]
pub type R_COEFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R_COEFF` writer - Coefficient that multiplies the R component NOTE: not used if FORMAT == BYPASS"]
pub type R_COEFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAM_CFG_FILTER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Coefficient that multiplies the B component NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    pub fn b_coeff(&self) -> B_COEFF_R {
        B_COEFF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Coefficient that multiplies the G component NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    pub fn g_coeff(&self) -> G_COEFF_R {
        G_COEFF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Coefficient that multiplies the R component NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    pub fn r_coeff(&self) -> R_COEFF_R {
        R_COEFF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Coefficient that multiplies the B component NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    #[must_use]
    pub fn b_coeff(&mut self) -> B_COEFF_W<0> {
        B_COEFF_W::new(self)
    }
    #[doc = "Bits 8:15 - Coefficient that multiplies the G component NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    #[must_use]
    pub fn g_coeff(&mut self) -> G_COEFF_W<8> {
        G_COEFF_W::new(self)
    }
    #[doc = "Bits 16:23 - Coefficient that multiplies the R component NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    #[must_use]
    pub fn r_coeff(&mut self) -> R_COEFF_W<16> {
        R_COEFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RGB coefficients configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg_filter](index.html) module"]
pub struct CAM_CFG_FILTER_SPEC;
impl crate::RegisterSpec for CAM_CFG_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg_filter::R](R) reader structure"]
impl crate::Readable for CAM_CFG_FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg_filter::W](W) writer structure"]
impl crate::Writable for CAM_CFG_FILTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CFG_FILTER to value 0"]
impl crate::Resettable for CAM_CFG_FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
