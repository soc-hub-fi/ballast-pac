#[doc = "Register `CLUSTER_CTRL` reader"]
pub struct R(crate::R<CLUSTER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLUSTER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLUSTER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLUSTER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLUSTER_CTRL` writer"]
pub struct W(crate::W<CLUSTER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUSTER_CTRL_SPEC>;
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
impl From<crate::W<CLUSTER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUSTER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bypass` reader - "]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `bypass` writer - "]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLUSTER_CTRL_SPEC, bool, O>;
#[doc = "Field `POW` reader - "]
pub type POW_R = crate::BitReader<bool>;
#[doc = "Field `POW` writer - "]
pub type POW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLUSTER_CTRL_SPEC, bool, O>;
#[doc = "Field `fetch_enable` reader - "]
pub type FETCH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `fetch_enable` writer - "]
pub type FETCH_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLUSTER_CTRL_SPEC, bool, O>;
#[doc = "Field `rstn` reader - "]
pub type RSTN_R = crate::BitReader<bool>;
#[doc = "Field `rstn` writer - "]
pub type RSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLUSTER_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pow(&self) -> POW_R {
        POW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fetch_enable(&self) -> FETCH_ENABLE_R {
        FETCH_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rstn(&self) -> RSTN_R {
        RSTN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pow(&mut self) -> POW_W<1> {
        POW_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn fetch_enable(&mut self) -> FETCH_ENABLE_W<2> {
        FETCH_ENABLE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rstn(&mut self) -> RSTN_W<3> {
        RSTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cluster_ctrl](index.html) module"]
pub struct CLUSTER_CTRL_SPEC;
impl crate::RegisterSpec for CLUSTER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cluster_ctrl::R](R) reader structure"]
impl crate::Readable for CLUSTER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cluster_ctrl::W](W) writer structure"]
impl crate::Writable for CLUSTER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLUSTER_CTRL to value 0x09"]
impl crate::Resettable for CLUSTER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
