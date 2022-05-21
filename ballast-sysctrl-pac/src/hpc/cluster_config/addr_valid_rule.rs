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
pub struct ADDR_VALID_RULE_R(crate::FieldReader<u64, u64>);
impl ADDR_VALID_RULE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        ADDR_VALID_RULE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_VALID_RULE_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `addr_valid_rule` writer - "]
pub struct ADDR_VALID_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_VALID_RULE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
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
    pub fn addr_valid_rule(&mut self) -> ADDR_VALID_RULE_W {
        ADDR_VALID_RULE_W { w: self }
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
}
#[doc = "`reset()` method sets addr_valid_rule to value 0"]
impl crate::Resettable for ADDR_VALID_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
