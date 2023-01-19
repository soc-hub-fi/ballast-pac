#[doc = "Register `CFG_RD_WEIGHT_2` reader"]
pub struct R(crate::R<CFG_RD_WEIGHT_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_RD_WEIGHT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_RD_WEIGHT_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_RD_WEIGHT_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_RD_WEIGHT_2` writer"]
pub struct W(crate::W<CFG_RD_WEIGHT_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_RD_WEIGHT_2_SPEC>;
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
impl From<crate::W<CFG_RD_WEIGHT_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_RD_WEIGHT_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_WEIGHT_CDMA_WT` reader - "]
pub type RD_WEIGHT_CDMA_WT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_WEIGHT_RBK` reader - "]
pub type RD_WEIGHT_RBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_WEIGHT_RSV_1` reader - "]
pub type RD_WEIGHT_RSV_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_WEIGHT_RSV_0` reader - "]
pub type RD_WEIGHT_RSV_0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_weight_cdma_wt(&self) -> RD_WEIGHT_CDMA_WT_R {
        RD_WEIGHT_CDMA_WT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rd_weight_rbk(&self) -> RD_WEIGHT_RBK_R {
        RD_WEIGHT_RBK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rd_weight_rsv_1(&self) -> RD_WEIGHT_RSV_1_R {
        RD_WEIGHT_RSV_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_weight_rsv_0(&self) -> RD_WEIGHT_RSV_0_R {
        RD_WEIGHT_RSV_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register2 to control the read weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_rd_weight_2](index.html) module"]
pub struct CFG_RD_WEIGHT_2_SPEC;
impl crate::RegisterSpec for CFG_RD_WEIGHT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_rd_weight_2::R](R) reader structure"]
impl crate::Readable for CFG_RD_WEIGHT_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_rd_weight_2::W](W) writer structure"]
impl crate::Writable for CFG_RD_WEIGHT_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_RD_WEIGHT_2 to value 0x0100_0100"]
impl crate::Resettable for CFG_RD_WEIGHT_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
