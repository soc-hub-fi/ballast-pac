#[doc = "Register `CFG_WR_WEIGHT_1` reader"]
pub struct R(crate::R<CFG_WR_WEIGHT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_WR_WEIGHT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_WR_WEIGHT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_WR_WEIGHT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_WR_WEIGHT_1` writer"]
pub struct W(crate::W<CFG_WR_WEIGHT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_WR_WEIGHT_1_SPEC>;
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
impl From<crate::W<CFG_WR_WEIGHT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_WR_WEIGHT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_WEIGHT_RBK` reader - "]
pub type WR_WEIGHT_RBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_WEIGHT_RSV_2` reader - "]
pub type WR_WEIGHT_RSV_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_WEIGHT_RSV_1` reader - "]
pub type WR_WEIGHT_RSV_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_WEIGHT_RSV_0` reader - "]
pub type WR_WEIGHT_RSV_0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wr_weight_rbk(&self) -> WR_WEIGHT_RBK_R {
        WR_WEIGHT_RBK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wr_weight_rsv_2(&self) -> WR_WEIGHT_RSV_2_R {
        WR_WEIGHT_RSV_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn wr_weight_rsv_1(&self) -> WR_WEIGHT_RSV_1_R {
        WR_WEIGHT_RSV_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wr_weight_rsv_0(&self) -> WR_WEIGHT_RSV_0_R {
        WR_WEIGHT_RSV_0_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "Register1 to control the write weight of clients in MCIF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_wr_weight_1](index.html) module"]
pub struct CFG_WR_WEIGHT_1_SPEC;
impl crate::RegisterSpec for CFG_WR_WEIGHT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_wr_weight_1::R](R) reader structure"]
impl crate::Readable for CFG_WR_WEIGHT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_wr_weight_1::W](W) writer structure"]
impl crate::Writable for CFG_WR_WEIGHT_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_WR_WEIGHT_1 to value 0x0100_0100"]
impl crate::Resettable for CFG_WR_WEIGHT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
