#[doc = "Register `RX_IFG_SET` reader"]
pub struct R(crate::R<RX_IFG_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_IFG_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_IFG_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_IFG_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_IFG_SET` writer"]
pub struct W(crate::W<RX_IFG_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_IFG_SET_SPEC>;
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
impl From<crate::W<RX_IFG_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_IFG_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_IFG_SET` reader - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub type RX_IFG_SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_IFG_SET` writer - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub type RX_IFG_SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_IFG_SET_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    #[inline(always)]
    pub fn rx_ifg_set(&self) -> RX_IFG_SET_R {
        RX_IFG_SET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ifg_set(&mut self) -> RX_IFG_SET_W<0> {
        RX_IFG_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ifg_set](index.html) module"]
pub struct RX_IFG_SET_SPEC;
impl crate::RegisterSpec for RX_IFG_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ifg_set::R](R) reader structure"]
impl crate::Readable for RX_IFG_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ifg_set::W](W) writer structure"]
impl crate::Writable for RX_IFG_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_IFG_SET to value 0x0c"]
impl crate::Resettable for RX_IFG_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
