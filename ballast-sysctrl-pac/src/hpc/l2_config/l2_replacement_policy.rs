#[doc = "Register `l2_replacement_policy` reader"]
pub struct R(crate::R<L2_REPLACEMENT_POLICY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_REPLACEMENT_POLICY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_REPLACEMENT_POLICY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_REPLACEMENT_POLICY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l2_replacement_policy` writer"]
pub struct W(crate::W<L2_REPLACEMENT_POLICY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2_REPLACEMENT_POLICY_SPEC>;
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
impl From<crate::W<L2_REPLACEMENT_POLICY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2_REPLACEMENT_POLICY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l2_replacement_policy` reader - "]
pub type L2_REPLACEMENT_POLICY_R = crate::FieldReader<u64, u64>;
#[doc = "Field `l2_replacement_policy` writer - "]
pub type L2_REPLACEMENT_POLICY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, L2_REPLACEMENT_POLICY_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn l2_replacement_policy(&self) -> L2_REPLACEMENT_POLICY_R {
        L2_REPLACEMENT_POLICY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn l2_replacement_policy(&mut self) -> L2_REPLACEMENT_POLICY_W<0> {
        L2_REPLACEMENT_POLICY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select L2 core priority algorithm; NI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_replacement_policy](index.html) module"]
pub struct L2_REPLACEMENT_POLICY_SPEC;
impl crate::RegisterSpec for L2_REPLACEMENT_POLICY_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [l2_replacement_policy::R](R) reader structure"]
impl crate::Readable for L2_REPLACEMENT_POLICY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2_replacement_policy::W](W) writer structure"]
impl crate::Writable for L2_REPLACEMENT_POLICY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets l2_replacement_policy to value 0"]
impl crate::Resettable for L2_REPLACEMENT_POLICY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
