#[doc = "Register `addr_valid_rule` reader"]
pub struct R(crate::R<ADDR_VALID_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_VALID_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_VALID_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_VALID_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `addr_valid_rule` writer"]
pub struct W(crate::W<ADDR_VALID_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_VALID_RULE_SPEC>;
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
impl From<crate::W<ADDR_VALID_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_VALID_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `addr_valid_rule` reader - "]
pub type ADDR_VALID_RULE_R = crate::FieldReader<u64, u64>;
#[doc = "Field `addr_valid_rule` writer - "]
pub type ADDR_VALID_RULE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, ADDR_VALID_RULE_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn addr_valid_rule(&self) -> ADDR_VALID_RULE_R {
        ADDR_VALID_RULE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn addr_valid_rule(&mut self) -> ADDR_VALID_RULE_W<0> {
        ADDR_VALID_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Valid address space flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_valid_rule](index.html) module"]
pub struct ADDR_VALID_RULE_SPEC;
impl crate::RegisterSpec for ADDR_VALID_RULE_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [addr_valid_rule::R](R) reader structure"]
impl crate::Readable for ADDR_VALID_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_valid_rule::W](W) writer structure"]
impl crate::Writable for ADDR_VALID_RULE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets addr_valid_rule to value 0"]
impl crate::Resettable for ADDR_VALID_RULE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
