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
#[doc = "Field `PRODUCER` reader - "]
pub type PRODUCER_R = crate::BitReader<PRODUCER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRODUCER_A {
    #[doc = "0: `0`"]
    GROUP_0 = 0,
    #[doc = "1: `1`"]
    GROUP_1 = 1,
}
impl From<PRODUCER_A> for bool {
    #[inline(always)]
    fn from(variant: PRODUCER_A) -> Self {
        variant as u8 != 0
    }
}
impl PRODUCER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRODUCER_A {
        match self.bits {
            false => PRODUCER_A::GROUP_0,
            true => PRODUCER_A::GROUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `GROUP_0`"]
    #[inline(always)]
    pub fn is_group_0(&self) -> bool {
        *self == PRODUCER_A::GROUP_0
    }
    #[doc = "Checks if the value of the field is `GROUP_1`"]
    #[inline(always)]
    pub fn is_group_1(&self) -> bool {
        *self == PRODUCER_A::GROUP_1
    }
}
#[doc = "Field `PRODUCER` writer - "]
pub type PRODUCER_W<'a, const O: u8> = crate::BitWriter<'a, u32, S_POINTER_SPEC, PRODUCER_A, O>;
impl<'a, const O: u8> PRODUCER_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn group_0(self) -> &'a mut W {
        self.variant(PRODUCER_A::GROUP_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn group_1(self) -> &'a mut W {
        self.variant(PRODUCER_A::GROUP_1)
    }
}
#[doc = "Field `CONSUMER` reader - "]
pub type CONSUMER_R = crate::BitReader<CONSUMER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CONSUMER_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CONSUMER_A::GROUP_1
    }
    #[doc = "Checks if the value of the field is `GROUP_0`"]
    #[inline(always)]
    pub fn is_group_0(&self) -> bool {
        *self == CONSUMER_A::GROUP_0
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
    #[must_use]
    pub fn producer(&mut self) -> PRODUCER_W<0> {
        PRODUCER_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_POINTER to value 0"]
impl crate::Resettable for S_POINTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
