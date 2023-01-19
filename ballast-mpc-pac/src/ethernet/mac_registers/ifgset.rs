#[doc = "Register `IFGset` reader"]
pub struct R(crate::R<IFGSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFGSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFGSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFGSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFGset` writer"]
pub struct W(crate::W<IFGSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFGSET_SPEC>;
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
impl From<crate::W<IFGSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFGSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFGset` reader - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub type IFGSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFGset` writer - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub type IFGSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFGSET_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    #[inline(always)]
    pub fn ifgset(&self) -> IFGSET_R {
        IFGSET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    #[inline(always)]
    #[must_use]
    pub fn ifgset(&mut self) -> IFGSET_W<0> {
        IFGSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifgset](index.html) module"]
pub struct IFGSET_SPEC;
impl crate::RegisterSpec for IFGSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifgset::R](R) reader structure"]
impl crate::Readable for IFGSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifgset::W](W) writer structure"]
impl crate::Writable for IFGSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFGset to value 0x0c"]
impl crate::Resettable for IFGSET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
