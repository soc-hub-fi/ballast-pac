#[doc = "Register `S_POINTER` reader"]
pub struct R(crate::R<S_POINTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_POINTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_POINTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_POINTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_POINTER` writer"]
pub struct W(crate::W<S_POINTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_POINTER_SPEC>;
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
impl From<crate::W<S_POINTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_POINTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRODUCER_A {
    #[doc = "1: `1`"]
    GROUP_1 = 1,
    #[doc = "0: `0`"]
    GROUP_0 = 0,
}
impl From<PRODUCER_A> for bool {
    #[inline(always)]
    fn from(variant: PRODUCER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRODUCER` reader - "]
pub struct PRODUCER_R(crate::FieldReader<bool>);
impl PRODUCER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRODUCER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRODUCER_A {
        match self.bits {
            true => PRODUCER_A::GROUP_1,
            false => PRODUCER_A::GROUP_0,
        }
    }
    #[doc = "Checks if the value of the field is `GROUP_1`"]
    #[inline(always)]
    pub fn is_group_1(&self) -> bool {
        **self == PRODUCER_A::GROUP_1
    }
    #[doc = "Checks if the value of the field is `GROUP_0`"]
    #[inline(always)]
    pub fn is_group_0(&self) -> bool {
        **self == PRODUCER_A::GROUP_0
    }
}
impl core::ops::Deref for PRODUCER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRODUCER` writer - "]
pub struct PRODUCER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRODUCER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRODUCER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn group_1(self) -> &'a mut W {
        self.variant(PRODUCER_A::GROUP_1)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn group_0(self) -> &'a mut W {
        self.variant(PRODUCER_A::GROUP_0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONSUMER_A {
    #[doc = "1: `1`"]
    GROUP_1 = 1,
    #[doc = "0: `0`"]
    GROUP_0 = 0,
}
impl From<CONSUMER_A> for bool {
    #[inline(always)]
    fn from(variant: CONSUMER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONSUMER` reader - "]
pub struct CONSUMER_R(crate::FieldReader<bool>);
impl CONSUMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONSUMER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONSUMER_A {
        match self.bits {
            true => CONSUMER_A::GROUP_1,
            false => CONSUMER_A::GROUP_0,
        }
    }
    #[doc = "Checks if the value of the field is `GROUP_1`"]
    #[inline(always)]
    pub fn is_group_1(&self) -> bool {
        **self == CONSUMER_A::GROUP_1
    }
    #[doc = "Checks if the value of the field is `GROUP_0`"]
    #[inline(always)]
    pub fn is_group_0(&self) -> bool {
        **self == CONSUMER_A::GROUP_0
    }
}
impl core::ops::Deref for CONSUMER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn producer(&self) -> PRODUCER_R {
        PRODUCER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn consumer(&self) -> CONSUMER_R {
        CONSUMER_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn producer(&mut self) -> PRODUCER_W {
        PRODUCER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pointer for CSB master and data path to access groups\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_pointer](index.html) module"]
pub struct S_POINTER_SPEC;
impl crate::RegisterSpec for S_POINTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_pointer::R](R) reader structure"]
impl crate::Readable for S_POINTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_pointer::W](W) writer structure"]
impl crate::Writable for S_POINTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_POINTER to value 0"]
impl crate::Resettable for S_POINTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
